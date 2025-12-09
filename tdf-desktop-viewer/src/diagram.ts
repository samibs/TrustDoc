import { DiagramBlock } from 'tdf-ts';

export function renderDiagram(diagram: DiagramBlock): HTMLElement {
    const container = document.createElement('div');
    container.className = 'diagram-container';
    if (diagram.id) container.id = diagram.id;

    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    svg.setAttribute('width', '100%');
    svg.setAttribute('height', '400');
    svg.setAttribute('viewBox', '0 0 800 400');

    // Simple hierarchical layout
    if (diagram.diagram_type === 'hierarchical') {
        renderHierarchicalDiagram(svg, diagram);
    } else {
        // Default: simple node-edge rendering
        renderSimpleDiagram(svg, diagram);
    }

    container.appendChild(svg);
    return container;
}

function renderHierarchicalDiagram(svg: SVGElement, diagram: DiagramBlock) {
    const nodeWidth = 120;
    const nodeHeight = 60;
    const spacing = 40;
    
    // Calculate positions
    const positions = new Map<string, { x: number; y: number }>();
    const levels: string[][] = [];
    
    // Simple layout: first node at top, others below
    if (diagram.nodes.length > 0) {
        const root = diagram.nodes[0];
        positions.set(root.id, { x: 400, y: 50 });
        levels.push([root.id]);
        
        let y = 150;
        for (let i = 1; i < diagram.nodes.length; i++) {
            const node = diagram.nodes[i];
            const x = 200 + (i - 1) * (nodeWidth + spacing);
            positions.set(node.id, { x, y });
        }
    }

    // Render edges
    for (const edge of diagram.edges) {
        const from = positions.get(edge.from);
        const to = positions.get(edge.to);
        if (from && to) {
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('x1', from.x.toString());
            line.setAttribute('y1', (from.y + nodeHeight / 2).toString());
            line.setAttribute('x2', to.x.toString());
            line.setAttribute('y2', (to.y + nodeHeight / 2).toString());
            line.setAttribute('stroke', '#333');
            line.setAttribute('stroke-width', '2');
            if (edge.type === 'dashed') {
                line.setAttribute('stroke-dasharray', '5,5');
            }
            svg.appendChild(line);
        }
    }

    // Render nodes
    for (const node of diagram.nodes) {
        const pos = positions.get(node.id);
        if (pos) {
            const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
            rect.setAttribute('x', (pos.x - nodeWidth / 2).toString());
            rect.setAttribute('y', (pos.y - nodeHeight / 2).toString());
            rect.setAttribute('width', nodeWidth.toString());
            rect.setAttribute('height', nodeHeight.toString());
            rect.setAttribute('fill', '#fff');
            rect.setAttribute('stroke', '#333');
            rect.setAttribute('stroke-width', '2');
            if (node.shape === 'rounded') {
                rect.setAttribute('rx', '5');
            }
            svg.appendChild(rect);

            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('x', pos.x.toString());
            text.setAttribute('y', pos.y.toString());
            text.setAttribute('text-anchor', 'middle');
            text.setAttribute('dominant-baseline', 'middle');
            text.setAttribute('font-size', '12');
            text.textContent = node.label;
            svg.appendChild(text);
        }
    }
}

function renderSimpleDiagram(svg: SVGElement, diagram: DiagramBlock) {
    // Simple circular layout
    const centerX = 400;
    const centerY = 200;
    const radius = 150;
    const angleStep = (2 * Math.PI) / diagram.nodes.length;

    const positions = new Map<string, { x: number; y: number }>();
    
    diagram.nodes.forEach((node, i) => {
        const angle = i * angleStep;
        const x = centerX + radius * Math.cos(angle);
        const y = centerY + radius * Math.sin(angle);
        positions.set(node.id, { x, y });
    });

    // Render edges
    for (const edge of diagram.edges) {
        const from = positions.get(edge.from);
        const to = positions.get(edge.to);
        if (from && to) {
            const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
            line.setAttribute('x1', from.x.toString());
            line.setAttribute('y1', from.y.toString());
            line.setAttribute('x2', to.x.toString());
            line.setAttribute('y2', to.y.toString());
            line.setAttribute('stroke', '#333');
            line.setAttribute('stroke-width', '2');
            svg.appendChild(line);
        }
    }

    // Render nodes
    for (const node of diagram.nodes) {
        const pos = positions.get(node.id);
        if (pos) {
            const circle = document.createElementNS('http://www.w3.org/2000/svg', 'circle');
            circle.setAttribute('cx', pos.x.toString());
            circle.setAttribute('cy', pos.y.toString());
            circle.setAttribute('r', '30');
            circle.setAttribute('fill', '#fff');
            circle.setAttribute('stroke', '#333');
            circle.setAttribute('stroke-width', '2');
            svg.appendChild(circle);

            const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
            text.setAttribute('x', pos.x.toString());
            text.setAttribute('y', pos.y.toString());
            text.setAttribute('text-anchor', 'middle');
            text.setAttribute('dominant-baseline', 'middle');
            text.setAttribute('font-size', '10');
            text.textContent = node.label;
            svg.appendChild(text);
        }
    }
}

