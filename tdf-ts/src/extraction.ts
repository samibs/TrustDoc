import { TdfDocument, TableBlock, CellValue } from './document';

export interface ExtractedData {
  metadata: {
    title: string;
    id: string;
    created: string;
    modified: string;
  };
  tables: {
    [id: string]: {
      columns: string[];
      rows: any[][];
    };
  };
  metrics?: {
    [key: string]: number;
  };
}

export function extractData(document: TdfDocument): ExtractedData {
  const extracted: ExtractedData = {
    metadata: {
      title: document.manifest.document.title,
      id: document.manifest.document.id,
      created: document.manifest.document.created,
      modified: document.manifest.document.modified,
    },
    tables: {},
  };

  // Extract tables
  for (const section of document.content.sections) {
    for (const block of section.content) {
      if (block.type === 'table') {
        const table = block as TableBlock;
        const columns = table.columns.map(col => col.id);
        const rows: any[][] = [];

        for (const row of table.rows) {
          const rowData: any[] = [];
          for (const colId of columns) {
            const cell = row[colId];
            if (cell) {
              // Extract raw value if available
              if ('raw' in cell) {
                rowData.push((cell as any).raw);
              } else if ('value' in cell) {
                rowData.push((cell as any).value);
              } else {
                rowData.push(null);
              }
            } else {
              rowData.push(null);
            }
          }
          rows.push(rowData);
        }

        extracted.tables[table.id] = {
          columns,
          rows,
        };
      }
    }
  }

  // Use data.json if available
  if (document.data) {
    if (document.data.metrics) {
      extracted.metrics = document.data.metrics;
    }
  }

  return extracted;
}

