#!/bin/bash

count=$(cat ~/.local/share/akari/todos.json 2>/dev/null | python3 -c "
import sys, json
todos = json.load(sys.stdin)
print(len([t for t in todos if not t['done']]))" 2>/dev/null || echo 0)

echo "📝 $count"
