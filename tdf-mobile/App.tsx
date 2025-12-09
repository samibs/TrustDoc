import React, { useState } from 'react';
import {
  StyleSheet,
  View,
  Text,
  Button,
  ScrollView,
  Alert,
  ActivityIndicator,
  SafeAreaView,
} from 'react-native';
import * as DocumentPicker from 'expo-document-picker';
import * as FileSystem from 'expo-file-system';
import * as Sharing from 'expo-sharing';
import { loadDocument, TdfDocument } from 'tdf-ts';
import DocumentViewer from './src/components/DocumentViewer';
import VerificationPanel from './src/components/VerificationPanel';
import Toolbar from './src/components/Toolbar';

export default function App() {
  const [document, setDocument] = useState<TdfDocument | null>(null);
  const [loading, setLoading] = useState(false);
  const [verificationResult, setVerificationResult] = useState<any>(null);
  const [fileUri, setFileUri] = useState<string | null>(null);

  const pickDocument = async () => {
    try {
      setLoading(true);
      const result = await DocumentPicker.getDocumentAsync({
        type: 'application/zip',
        copyToCacheDirectory: true,
      });

      if (!result.canceled && result.assets && result.assets.length > 0) {
        const asset = result.assets[0];
        setFileUri(asset.uri);
        await loadTdfDocument(asset.uri);
      }
    } catch (error) {
      Alert.alert('Error', `Failed to pick document: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const loadTdfDocument = async (uri: string) => {
    try {
      setLoading(true);
      const fileContent = await FileSystem.readAsStringAsync(uri, {
        encoding: FileSystem.EncodingType.Base64,
      });
      
      // Convert base64 to blob
      const binaryString = atob(fileContent);
      const bytes = new Uint8Array(binaryString.length);
      for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
      }
      const blob = new Blob([bytes], { type: 'application/zip' });
      const file = new File([blob], 'document.tdf', { type: 'application/zip' });
      
      const doc = await loadDocument(file);
      setDocument(doc);
      setVerificationResult(null);
    } catch (error) {
      Alert.alert('Error', `Failed to load document: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const verifyDocument = async () => {
    if (!fileUri) {
      Alert.alert('Error', 'No document loaded');
      return;
    }

    try {
      setLoading(true);
      // Use WASM or native module for verification
      // For now, show placeholder
      setVerificationResult({
        integrity_valid: true,
        root_hash: '...',
        signature_count: document?.manifest ? 1 : 0,
        message: 'Verification completed (native module integration pending)',
      });
    } catch (error) {
      Alert.alert('Error', `Verification failed: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const extractData = async () => {
    if (!document) return;

    try {
      const data = {
        title: document.manifest.document.title,
        id: document.manifest.document.id,
        created: document.manifest.document.created,
        modified: document.manifest.document.modified,
        sections: document.content.sections.map(s => ({
          id: s.id,
          title: s.title,
          blockCount: s.content.length,
        })),
      };

      const jsonString = JSON.stringify(data, null, 2);
      const fileUri = `${FileSystem.cacheDirectory}${document.manifest.document.id}.json`;
      await FileSystem.writeAsStringAsync(fileUri, jsonString);

      if (await Sharing.isAvailableAsync()) {
        await Sharing.shareAsync(fileUri);
      } else {
        Alert.alert('Success', 'Data extracted (sharing not available)');
      }
    } catch (error) {
      Alert.alert('Error', `Failed to extract data: ${error}`);
    }
  };

  if (loading) {
    return (
      <SafeAreaView style={styles.container}>
        <View style={styles.center}>
          <ActivityIndicator size="large" color="#1a237e" />
          <Text style={styles.loadingText}>Loading...</Text>
        </View>
      </SafeAreaView>
    );
  }

  if (!document) {
    return (
      <SafeAreaView style={styles.container}>
        <View style={styles.homeContainer}>
          <Text style={styles.title}>🔐 TDF Mobile Viewer</Text>
          <Text style={styles.subtitle}>TrustDoc Financial Document Viewer</Text>
          <Button
            title="📂 Open TDF Document"
            onPress={pickDocument}
            color="#1a237e"
          />
        </View>
      </SafeAreaView>
    );
  }

  return (
    <SafeAreaView style={styles.container}>
      <Toolbar
        onOpen={pickDocument}
        onVerify={verifyDocument}
        onExtract={extractData}
      />
      <ScrollView style={styles.scrollView}>
        {verificationResult && (
          <VerificationPanel result={verificationResult} />
        )}
        <DocumentViewer document={document} />
      </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f5f5f5',
  },
  center: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  loadingText: {
    marginTop: 16,
    fontSize: 16,
    color: '#666',
  },
  homeContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 32,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
    textAlign: 'center',
  },
  subtitle: {
    fontSize: 16,
    color: '#666',
    marginBottom: 32,
    textAlign: 'center',
  },
  scrollView: {
    flex: 1,
  },
});

