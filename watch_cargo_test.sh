
function build {
    clear
    cargo build 2>&1 | head -n 40
    return "${PIPESTATUS[0]}"
}

function test_it {
    clear
    cargo test
}

function build_run {
    build
    [ $? -eq 0 ] && test_it
}

#

build_run

while true
do
    inotifywait --recursive --event modify ./src 2>/dev/null && build_run
done
