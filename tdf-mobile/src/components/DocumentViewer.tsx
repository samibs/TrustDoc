import React from 'react';
import { View, Text, ScrollView, StyleSheet } from 'react-native';
import { TdfDocument } from 'tdf-ts';

interface DocumentViewerProps {
  document: TdfDocument;
}

export default function DocumentViewer({ document }: DocumentViewerProps) {
  return (
    <View style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>{document.manifest.document.title}</Text>
        <Text style={styles.subtitle}>TrustDoc Financial Format</Text>
        <View style={styles.badge}>
          <Text style={styles.badgeText}>SECURED & VERIFIED</Text>
        </View>
      </View>

      <View style={styles.meta}>
        <Text style={styles.metaText}>
          <Text style={styles.metaLabel}>Document ID: </Text>
          {document.manifest.document.id}
        </Text>
        <Text style={styles.metaText}>
          <Text style={styles.metaLabel}>Created: </Text>
          {new Date(document.manifest.document.created).toLocaleString()}
        </Text>
        <Text style={styles.metaText}>
          <Text style={styles.metaLabel}>Modified: </Text>
          {new Date(document.manifest.document.modified).toLocaleString()}
        </Text>
      </View>

      {document.manifest.authors.length > 0 && (
        <View style={styles.authors}>
          <Text style={styles.authorsText}>
            <Text style={styles.metaLabel}>Authors: </Text>
            {document.manifest.authors.map(a => a.name).join(', ')}
          </Text>
        </View>
      )}

      {document.content.sections.map((section, idx) => (
        <View key={idx} style={styles.section}>
          {section.title && (
            <Text style={styles.sectionTitle}>{section.title}</Text>
          )}
          {section.content.map((block, blockIdx) => (
            <View key={blockIdx} style={styles.block}>
              {renderBlock(block)}
            </View>
          ))}
        </View>
      ))}

      <View style={styles.footer}>
        <Text style={styles.footerText}>
          TrustDoc Financial Format - Confidential Business Document
        </Text>
        <Text style={styles.securityNote}>
          This document is cryptographically secured. Any tampering will be immediately detectable.
        </Text>
      </View>
    </View>
  );
}

function renderBlock(block: any): React.ReactNode {
  switch (block.type) {
    case 'heading':
      const Heading = `h${block.level}` as keyof JSX.IntrinsicElements;
      return <Heading style={styles[`heading${block.level}`]}>{block.text}</Heading>;

    case 'paragraph':
      return <Text style={styles.paragraph}>{block.text}</Text>;

    case 'list':
      return (
        <View style={styles.list}>
          {block.items.map((item: string, idx: number) => (
            <Text key={idx} style={styles.listItem}>
              {block.ordered ? `${idx + 1}. ` : '• '}{item}
            </Text>
          ))}
        </View>
      );

    case 'table':
      return (
        <View style={styles.table}>
          <View style={styles.tableHeader}>
            {block.columns.map((col: any) => (
              <Text key={col.id} style={styles.tableHeaderCell}>
                {col.header}
              </Text>
            ))}
          </View>
          {block.rows.map((row: any, rowIdx: number) => (
            <View key={rowIdx} style={styles.tableRow}>
              {block.columns.map((col: any) => (
                <Text key={col.id} style={styles.tableCell}>
                  {row[col.id]?.display || row[col.id]?.value || ''}
                </Text>
              ))}
            </View>
          ))}
        </View>
      );

    case 'footnote':
      return <Text style={styles.footnote}>{block.text}</Text>;

    default:
      return <Text style={styles.unknown}>[Unsupported block type: {block.type}]</Text>;
  }
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
  header: {
    borderBottomWidth: 2,
    borderBottomColor: '#1a237e',
    paddingBottom: 16,
    marginBottom: 16,
  },
  title: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
  },
  subtitle: {
    fontSize: 14,
    color: '#666',
    marginBottom: 8,
  },
  badge: {
    backgroundColor: '#4caf50',
    paddingVertical: 4,
    paddingHorizontal: 8,
    borderRadius: 4,
    alignSelf: 'flex-start',
  },
  badgeText: {
    color: 'white',
    fontSize: 12,
    fontWeight: 'bold',
  },
  meta: {
    backgroundColor: '#f5f5f5',
    padding: 12,
    borderRadius: 4,
    marginBottom: 12,
  },
  metaText: {
    fontSize: 12,
    color: '#666',
    marginBottom: 4,
  },
  metaLabel: {
    fontWeight: '600',
    color: '#333',
  },
  authors: {
    marginBottom: 16,
  },
  authorsText: {
    fontSize: 12,
    color: '#666',
  },
  section: {
    marginBottom: 24,
  },
  sectionTitle: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 12,
    borderBottomWidth: 1,
    borderBottomColor: '#e0e0e0',
    paddingBottom: 8,
  },
  block: {
    marginBottom: 12,
  },
  heading1: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
  },
  heading2: {
    fontSize: 20,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
  },
  heading3: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
  },
  heading4: {
    fontSize: 16,
    fontWeight: 'bold',
    color: '#1a237e',
    marginBottom: 8,
  },
  paragraph: {
    fontSize: 14,
    lineHeight: 20,
    color: '#333',
    marginBottom: 8,
  },
  list: {
    marginLeft: 16,
    marginBottom: 8,
  },
  listItem: {
    fontSize: 14,
    lineHeight: 20,
    color: '#333',
    marginBottom: 4,
  },
  table: {
    borderWidth: 1,
    borderColor: '#e0e0e0',
    borderRadius: 4,
    marginVertical: 8,
    overflow: 'hidden',
  },
  tableHeader: {
    flexDirection: 'row',
    backgroundColor: '#1a237e',
  },
  tableHeaderCell: {
    flex: 1,
    padding: 8,
    color: 'white',
    fontWeight: 'bold',
    fontSize: 12,
    borderRightWidth: 1,
    borderRightColor: 'rgba(255,255,255,0.2)',
  },
  tableRow: {
    flexDirection: 'row',
    borderTopWidth: 1,
    borderTopColor: '#e0e0e0',
  },
  tableCell: {
    flex: 1,
    padding: 8,
    fontSize: 12,
    color: '#333',
    borderRightWidth: 1,
    borderRightColor: '#e0e0e0',
  },
  footnote: {
    fontSize: 12,
    color: '#666',
    fontStyle: 'italic',
    marginLeft: 16,
    marginTop: 4,
  },
  unknown: {
    fontSize: 12,
    color: '#999',
    fontStyle: 'italic',
  },
  footer: {
    marginTop: 24,
    paddingTop: 16,
    borderTopWidth: 1,
    borderTopColor: '#e0e0e0',
    alignItems: 'center',
  },
  footerText: {
    fontSize: 12,
    color: '#666',
    textAlign: 'center',
    marginBottom: 8,
  },
  securityNote: {
    fontSize: 11,
    color: '#4caf50',
    textAlign: 'center',
    backgroundColor: '#e8f5e9',
    padding: 8,
    borderRadius: 4,
    borderLeftWidth: 4,
    borderLeftColor: '#4caf50',
  },
});

