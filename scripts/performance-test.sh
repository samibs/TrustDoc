#!/bin/bash
set -e

echo "TDF Performance Testing"
echo "======================="

# Test document sizes
SIZES=(100 500 1000 5000 10000)  # KB

for size_kb in "${SIZES[@]}"; do
    echo ""
    echo "Testing with ${size_kb}KB document..."
    
    # Generate test document with large content
    python3 <<EOF
import json
import sys

size_kb = ${size_kb}
target_size = size_kb * 1024

# Create large content
content = "X" * 100  # Base paragraph
paragraphs = []
current_size = 0

while current_size < target_size:
    paragraphs.append({
        "type": "paragraph",
        "text": content + f" (paragraph {len(paragraphs) + 1})",
        "id": f"p-{len(paragraphs) + 1}"
    })
    current_size += len(json.dumps(paragraphs[-1]))

doc = {
    "title": f"Performance Test Document ({size_kb}KB)",
    "language": "en",
    "styles": "body { }",
    "sections": [{
        "id": "sec-1",
        "title": "Test Section",
        "content": paragraphs
    }]
}

with open(f"test-${size_kb}kb.json", "w") as f:
    json.dump(doc, f)

print(f"Generated test-${size_kb}kb.json")
EOF

    # Create TDF
    echo "  Creating TDF..."
    time_start=$(date +%s%N)
    ./target/release/tdf create "test-${size_kb}kb.json" -o "test-${size_kb}kb.tdf" 2>&1 | grep -v "Created"
    time_end=$(date +%s%N)
    create_time=$(( (time_end - time_start) / 1000000 ))
    
    # Verify
    echo "  Verifying..."
    time_start=$(date +%s%N)
    ./target/release/tdf verify "test-${size_kb}kb.tdf" 2>&1 | grep -E "(VALID|INVALID)" || true
    time_end=$(date +%s%N)
    verify_time=$(( (time_end - time_start) / 1000000 ))
    
    # Get file size
    tdf_size=$(stat -c%s "test-${size_kb}kb.tdf" 2>/dev/null || stat -f%z "test-${size_kb}kb.tdf" 2>/dev/null || echo "0")
    tdf_size_kb=$(( tdf_size / 1024 ))
    
    echo "  Results:"
    echo "    TDF size: ${tdf_size_kb}KB"
    echo "    Create time: ${create_time}ms"
    echo "    Verify time: ${verify_time}ms"
    
    # Cleanup
    rm -f "test-${size_kb}kb.json" "test-${size_kb}kb.tdf"
done

echo ""
echo "Performance testing complete!"

