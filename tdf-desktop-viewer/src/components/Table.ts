// Table Component - Enterprise-grade data table
// Supports sorting, filtering, and proper typography

export interface TableColumn {
    key: string;
    label: string;
    sortable?: boolean;
    filterable?: boolean;
    render?: (value: any, row: any) => string | HTMLElement;
    monospace?: boolean; // Use monospace font (for hashes, IDs)
    align?: 'left' | 'center' | 'right';
    width?: string;
}

export interface TableOptions {
    columns: TableColumn[];
    data: any[];
    sortable?: boolean;
    filterable?: boolean;
    pagination?: boolean;
    pageSize?: number;
    striped?: boolean;
    hoverable?: boolean;
    selectable?: boolean;
    onRowClick?: (row: any, index: number) => void;
    className?: string;
}

type SortDirection = 'asc' | 'desc' | null;

export class Table {
    private container: HTMLElement;
    private table: HTMLTableElement;
    private thead: HTMLTableSectionElement;
    private tbody: HTMLTableSectionElement;
    private options: Required<Omit<TableOptions, 'onRowClick' | 'className'>> & Pick<TableOptions, 'onRowClick' | 'className'>;
    private currentSort: { column: string; direction: SortDirection } = { column: '', direction: null };
    private currentFilters: Map<string, string> = new Map();
    private currentPage: number = 1;
    private filteredData: any[];

    constructor(container: HTMLElement, options: TableOptions) {
        this.container = container;
        this.options = {
            columns: options.columns,
            data: options.data,
            sortable: options.sortable !== false,
            filterable: options.filterable === true,
            pagination: options.pagination === true,
            pageSize: options.pageSize || 10,
            striped: options.striped !== false,
            hoverable: options.hoverable !== false,
            selectable: options.selectable === true,
            onRowClick: options.onRowClick,
            className: options.className || ''
        };

        this.filteredData = [...this.options.data];
        this.table = this.create();
        this.thead = this.createHeader();
        this.tbody = this.createBody();
        
        this.table.appendChild(this.thead);
        this.table.appendChild(this.tbody);
        
        this.container.appendChild(this.table);
    }

    private create(): HTMLTableElement {
        const table = document.createElement('table');
        table.className = `table ${this.options.className}`.trim();
        table.setAttribute('role', 'table');
        return table;
    }

    private createHeader(): HTMLTableSectionElement {
        const thead = document.createElement('thead');
        const headerRow = document.createElement('tr');
        headerRow.setAttribute('role', 'row');

        // Filter row (if filterable)
        if (this.options.filterable) {
            const filterRow = document.createElement('tr');
            filterRow.className = 'table-filter-row';
            filterRow.setAttribute('role', 'row');
            
            this.options.columns.forEach(column => {
                const filterCell = document.createElement('th');
                filterCell.className = 'table-filter-cell';
                
                if (column.filterable !== false) {
                    const filterInput = document.createElement('input');
                    filterInput.type = 'text';
                    filterInput.className = 'table-filter-input';
                    filterInput.setAttribute('aria-label', `Filter ${column.label}`);
                    filterInput.setAttribute('placeholder', `Filter ${column.label}...`);
                    filterInput.setAttribute('tabindex', '0');
                    
                    filterInput.addEventListener('input', (e) => {
                        const value = (e.target as HTMLInputElement).value;
                        if (value) {
                            this.currentFilters.set(column.key, value.toLowerCase());
                        } else {
                            this.currentFilters.delete(column.key);
                        }
                        this.applyFilters();
                    });
                    
                    filterCell.appendChild(filterInput);
                }
                
                filterRow.appendChild(filterCell);
            });
            
            thead.appendChild(filterRow);
        }

        // Header row
        this.options.columns.forEach(column => {
            const th = document.createElement('th');
            th.className = 'table-header-cell';
            th.setAttribute('role', 'columnheader');
            th.setAttribute('scope', 'col');
            
            if (column.width) {
                th.style.width = column.width;
            }
            
            if (column.align) {
                th.style.textAlign = column.align;
            }

            const headerContent = document.createElement('div');
            headerContent.className = 'table-header-content';
            headerContent.textContent = column.label;

            if (this.options.sortable && column.sortable !== false) {
                th.classList.add('table-sortable');
                headerContent.classList.add('table-sortable-content');
                
                const sortIcon = document.createElement('span');
                sortIcon.className = 'table-sort-icon';
                sortIcon.setAttribute('aria-hidden', 'true');
                headerContent.appendChild(sortIcon);
                
                th.addEventListener('click', () => {
                    this.sort(column.key);
                });
                
                th.setAttribute('tabindex', '0');
                th.setAttribute('role', 'button');
                th.setAttribute('aria-label', `Sort by ${column.label}`);
                
                th.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter' || e.key === ' ') {
                        e.preventDefault();
                        this.sort(column.key);
                    }
                });
            }

            th.appendChild(headerContent);
            headerRow.appendChild(th);
        });

        thead.appendChild(headerRow);
        return thead;
    }

    private createBody(): HTMLTableSectionElement {
        const tbody = document.createElement('tbody');
        tbody.setAttribute('role', 'rowgroup');
        this.renderRows(tbody);
        return tbody;
    }

    private renderRows(tbody: HTMLTableSectionElement): void {
        tbody.innerHTML = '';

        const displayData = this.getDisplayData();

        if (displayData.length === 0) {
            const emptyRow = document.createElement('tr');
            emptyRow.className = 'table-empty-row';
            const emptyCell = document.createElement('td');
            emptyCell.colSpan = this.options.columns.length;
            emptyCell.className = 'table-empty-cell';
            emptyCell.textContent = 'No data available';
            emptyRow.appendChild(emptyCell);
            tbody.appendChild(emptyRow);
            return;
        }

        displayData.forEach((row, index) => {
            const tr = document.createElement('tr');
            tr.setAttribute('role', 'row');
            
            if (this.options.striped && index % 2 === 1) {
                tr.classList.add('table-row-striped');
            }
            
            if (this.options.hoverable) {
                tr.classList.add('table-row-hoverable');
            }

            if (this.options.onRowClick) {
                tr.style.cursor = 'pointer';
                tr.addEventListener('click', () => {
                    this.options.onRowClick!(row, index);
                });
            }

            this.options.columns.forEach(column => {
                const td = document.createElement('td');
                td.className = 'table-cell';
                td.setAttribute('role', 'gridcell');
                
                if (column.align) {
                    td.style.textAlign = column.align;
                }
                
                if (column.monospace) {
                    td.classList.add('table-cell-monospace');
                }

                const value = row[column.key];
                if (column.render) {
                    const rendered = column.render(value, row);
                    if (rendered instanceof HTMLElement) {
                        td.appendChild(rendered);
                    } else {
                        td.innerHTML = rendered;
                    }
                } else {
                    td.textContent = value != null ? String(value) : '';
                }

                tr.appendChild(td);
            });

            tbody.appendChild(tr);
        });
    }

    private getDisplayData(): any[] {
        let data = [...this.filteredData];

        // Apply sorting
        if (this.currentSort.direction) {
            data.sort((a, b) => {
                const aVal = a[this.currentSort.column];
                const bVal = b[this.currentSort.column];
                
                let comparison = 0;
                if (aVal < bVal) comparison = -1;
                if (aVal > bVal) comparison = 1;
                
                return this.currentSort.direction === 'asc' ? comparison : -comparison;
            });
        }

        // Apply pagination
        if (this.options.pagination) {
            const start = (this.currentPage - 1) * this.options.pageSize;
            const end = start + this.options.pageSize;
            data = data.slice(start, end);
        }

        return data;
    }

    private sort(columnKey: string): void {
        if (this.currentSort.column === columnKey) {
            // Cycle: asc -> desc -> null
            if (this.currentSort.direction === 'asc') {
                this.currentSort.direction = 'desc';
            } else if (this.currentSort.direction === 'desc') {
                this.currentSort.direction = null;
                this.currentSort.column = '';
            } else {
                this.currentSort.direction = 'asc';
            }
        } else {
            this.currentSort.column = columnKey;
            this.currentSort.direction = 'asc';
        }

        this.updateSortIndicators();
        this.renderRows(this.tbody);
    }

    private updateSortIndicators(): void {
        const headers = this.thead.querySelectorAll('.table-header-cell');
        headers.forEach((header, index) => {
            const column = this.options.columns[index];
            const sortIcon = header.querySelector('.table-sort-icon');
            
            if (sortIcon) {
                sortIcon.textContent = '';
                if (this.currentSort.column === column.key) {
                    if (this.currentSort.direction === 'asc') {
                        sortIcon.textContent = '↑';
                    } else if (this.currentSort.direction === 'desc') {
                        sortIcon.textContent = '↓';
                    }
                }
            }
        });
    }

    private applyFilters(): void {
        this.filteredData = this.options.data.filter(row => {
            for (const [columnKey, filterValue] of this.currentFilters.entries()) {
                const cellValue = String(row[columnKey] || '').toLowerCase();
                if (!cellValue.includes(filterValue)) {
                    return false;
                }
            }
            return true;
        });

        this.currentPage = 1; // Reset to first page
        this.renderRows(this.tbody);
    }

    updateData(data: any[]): void {
        this.options.data = data;
        this.filteredData = [...data];
        this.currentFilters.clear();
        this.currentPage = 1;
        this.currentSort = { column: '', direction: null };
        
        // Clear filter inputs
        const filterInputs = this.table.querySelectorAll('.table-filter-input') as NodeListOf<HTMLInputElement>;
        filterInputs.forEach(input => input.value = '');
        
        this.updateSortIndicators();
        this.renderRows(this.tbody);
    }

    getElement(): HTMLTableElement {
        return this.table;
    }
}
