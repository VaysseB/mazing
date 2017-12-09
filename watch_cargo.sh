CMD="clear ; cargo run 2>&1 | head -n 40"

sh -c "$CMD"

while true
do 
    touch ./lastwatch
    sleep 3
    find src -cnewer ./lastwatch -exec sh -c "$CMD" {} +
done
