CMD="clear ; cargo run | head -n 30"

sh -c "$CMD"

while true
do 
    touch ./lastwatch
    sleep 3
    find src -cnewer ./lastwatch -exec sh -c "$CMD" {} +
done
