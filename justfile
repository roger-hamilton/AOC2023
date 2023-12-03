work day part:
    cargo watch -w ./{{day}} \
        -x "clippy -p {{day}}" \
        -x "nextest run -p {{day}} {{part}}" \
        -x "run -p {{day}} --bin {{part}} -q" 
lint day:
    cargo clippy -p {{day}} -q
test day part: 
    cargo nextest run -p {{day}} {{part}}
run day part:
    cargo run -p {{day}} --bin {{part}} -q