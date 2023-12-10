lint day:
    cargo clippy -p {{day}}
test day:
    cargo test -p {{day}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day:
    cargo bench --bench {{day}}-bench >> {{day}}.bench.txt
create day:
    cargo generate --path ./template --name {{day}}
