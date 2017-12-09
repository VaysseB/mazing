CMD="clear ; cargo run 2>&1 | head -n 40"

sh -c "$CMD"

while true
do
    inotifywait --recursive --event modify ./src 2>/dev/null && sh -c "$CMD"
done
