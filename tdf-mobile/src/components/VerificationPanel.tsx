import React from 'react';
import { View, Text, StyleSheet } from 'react-native';

interface VerificationResult {
  integrity_valid: boolean;
  root_hash: string;
  signature_count: number;
  message?: string;
}

interface VerificationPanelProps {
  result: VerificationResult;
}

export default function VerificationPanel({ result }: VerificationPanelProps) {
  return (
    <View style={styles.container}>
      <Text style={styles.title}>🔍 Verification Results</Text>
      <View style={styles.resultItem}>
        <Text style={styles.label}>Integrity:</Text>
        <Text style={[styles.value, result.integrity_valid ? styles.valid : styles.invalid]}>
          {result.integrity_valid ? '✓ VALID' : '✗ INVALID'}
        </Text>
      </View>
      <View style={styles.resultItem}>
        <Text style={styles.label}>Root Hash:</Text>
        <Text style={styles.hash} numberOfLines={1} ellipsizeMode="middle">
          {result.root_hash}
        </Text>
      </View>
      <View style={styles.resultItem}>
        <Text style={styles.label}>Signatures:</Text>
        <Text style={styles.value}>{result.signature_count}</Text>
      </View>
      {result.message && (
        <Text style={styles.message}>{result.message}</Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: 'white',
    margin: 16,
    padding: 16,
    borderRadius: 8,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 4,
    elevation: 3,
  },
  title: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 12,
  },
  resultItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: 8,
    paddingVertical: 4,
  },
  label: {
    fontSize: 14,
    color: '#666',
    fontWeight: '500',
  },
  value: {
    fontSize: 14,
    color: '#333',
    fontWeight: '600',
  },
  valid: {
    color: '#388e3c',
  },
  invalid: {
    color: '#d32f2f',
  },
  hash: {
    fontSize: 12,
    fontFamily: 'monospace',
    color: '#666',
    flex: 1,
    textAlign: 'right',
    marginLeft: 8,
  },
  message: {
    marginTop: 8,
    fontSize: 12,
    color: '#666',
    fontStyle: 'italic',
  },
});

