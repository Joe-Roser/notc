find ./$1 -type f -print0 |
    while IFS= read -r -d '' f; do
      echo "===== $f ====="
      cat "$f"
      echo
    done | wl-copy

