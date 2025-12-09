import JSZip from 'jszip';

export interface TdfDocument {
  manifest: Manifest;
  content: DocumentContent;
  styles: string;
  layout?: Layout;
  data?: any;
}

export interface Manifest {
  schema_version: string;
  document: DocumentMeta;
  authors: Author[];
  classification?: Classification;
  integrity: IntegrityBlock;
}

export interface DocumentMeta {
  id: string;
  title: string;
  language: string;
  created: string;
  modified: string;
}

export interface Author {
  id: string;
  name: string;
  role?: string;
}

export type Classification = 'public' | 'internal' | 'confidential' | 'restricted';

export interface IntegrityBlock {
  root_hash: string;
  algorithm: 'sha256' | 'blake3';
}

export interface DocumentContent {
  sections: Section[];
}

export interface Section {
  id: string;
  title?: string;
  content: ContentBlock[];
}

export type ContentBlock =
  | HeadingBlock
  | ParagraphBlock
  | ListBlock
  | TableBlock
  | DiagramBlock
  | FigureBlock
  | FootnoteBlock;

export interface HeadingBlock {
  type: 'heading';
  level: number;
  text: string;
  id?: string;
}

export interface ParagraphBlock {
  type: 'paragraph';
  text: string;
  id?: string;
}

export interface ListBlock {
  type: 'list';
  ordered: boolean;
  items: string[];
  id?: string;
}

export interface TableBlock {
  type: 'table';
  id: string;
  caption?: string;
  columns: TableColumn[];
  rows: TableRow[];
  footer?: string[];
}

export interface TableColumn {
  id: string;
  header: string;
  type: CellType;
  currency?: string;
}

export interface TableRow {
  [key: string]: CellValue;
}

export type CellType = 'text' | 'number' | 'currency' | 'percentage' | 'date' | 'formula';

export type CellValue =
  | { type: 'text'; value: string }
  | { type: 'number'; raw: number; display: string }
  | { type: 'currency'; raw: number; display: string; currency: string }
  | { type: 'percentage'; raw: number; display: string }
  | { type: 'date'; raw: string; display: string };

export interface DiagramBlock {
  type: 'diagram';
  id: string;
  diagram_type: 'hierarchical' | 'flowchart' | 'relationship';
  title?: string;
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  layout?: DiagramLayout;
}

export interface DiagramNode {
  id: string;
  label: string;
  shape?: 'box' | 'circle' | 'diamond' | 'rounded';
  style?: string;
}

export interface DiagramEdge {
  from: string;
  to: string;
  type: 'solid' | 'dashed' | 'dotted';
  label?: string;
}

export interface DiagramLayout {
  direction?: 'top-down' | 'left-right' | 'bottom-up' | 'right-left';
  spacing?: 'compact' | 'normal' | 'wide';
}

export interface FigureBlock {
  type: 'figure';
  id: string;
  asset: string;
  alt: string;
  caption?: string;
  width?: number;
}

export interface FootnoteBlock {
  type: 'footnote';
  id: string;
  text: string;
}

export interface Layout {
  version: number;
  pages: PageLayout;
  elements: LayoutElement[];
}

export interface PageLayout {
  size: PageSize;
  orientation: 'portrait' | 'landscape';
  margins: Margins;
}

export type PageSize = 'A4' | 'Letter' | 'Legal' | { type: 'custom'; width: string; height: string };

export interface Margins {
  top: string;
  bottom: string;
  left: string;
  right: string;
}

export interface LayoutElement {
  ref: string;
  page: number;
  position: Position;
  size?: Size;
}

export interface Position {
  x: string;
  y: string;
}

export interface Size {
  width: string;
  height: string;
}

export async function loadDocument(file: File | Blob): Promise<TdfDocument> {
  const zip = await JSZip.loadAsync(file);
  
  // Read manifest
  const manifestFile = zip.file('manifest.cbor');
  if (!manifestFile) {
    throw new Error('Missing manifest.cbor in TDF file');
  }
  const manifestBytes = await manifestFile.async('uint8array');
  const manifest = await parseCbor(manifestBytes);

  // Read content
  const contentFile = zip.file('content.cbor');
  if (!contentFile) {
    throw new Error('Missing content.cbor in TDF file');
  }
  const contentBytes = await contentFile.async('uint8array');
  const content = await parseCbor(contentBytes);

  // Read styles
  const stylesFile = zip.file('styles.css');
  if (!stylesFile) {
    throw new Error('Missing styles.css in TDF file');
  }
  const styles = await stylesFile.async('string');

  // Read layout (optional)
  let layout: Layout | undefined;
  const layoutFile = zip.file('layout.cbor');
  if (layoutFile) {
    const layoutBytes = await layoutFile.async('uint8array');
    layout = await parseCbor(layoutBytes);
  }

  // Read data (optional)
  let data: any;
  const dataFile = zip.file('data.json');
  if (dataFile) {
    const dataStr = await dataFile.async('string');
    data = JSON.parse(dataStr);
  }

  return {
    manifest,
    content,
    styles,
    layout,
    data,
  };
}

// CBOR parser using cbor-web
async function parseCbor(bytes: Uint8Array): Promise<any> {
  const { decode } = await import('cbor-web');
  try {
    return decode(bytes);
  } catch (error) {
    throw new Error(`CBOR parsing failed: ${error}`);
  }
}

