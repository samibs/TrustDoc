import React from 'react';
import { View, TouchableOpacity, Text, StyleSheet } from 'react-native';

interface ToolbarProps {
  onOpen: () => void;
  onVerify: () => void;
  onExtract: () => void;
}

export default function Toolbar({ onOpen, onVerify, onExtract }: ToolbarProps) {
  return (
    <View style={styles.toolbar}>
      <TouchableOpacity style={styles.button} onPress={onOpen}>
        <Text style={styles.buttonText}>📂 Open</Text>
      </TouchableOpacity>
      <TouchableOpacity style={styles.button} onPress={onVerify}>
        <Text style={styles.buttonText}>🔍 Verify</Text>
      </TouchableOpacity>
      <TouchableOpacity style={styles.button} onPress={onExtract}>
        <Text style={styles.buttonText}>📊 Extract</Text>
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  toolbar: {
    flexDirection: 'row',
    backgroundColor: 'white',
    padding: 8,
    borderBottomWidth: 1,
    borderBottomColor: '#e0e0e0',
    justifyContent: 'space-around',
  },
  button: {
    backgroundColor: '#1a237e',
    paddingVertical: 8,
    paddingHorizontal: 16,
    borderRadius: 4,
    minWidth: 80,
    alignItems: 'center',
  },
  buttonText: {
    color: 'white',
    fontSize: 14,
    fontWeight: '500',
  },
});

