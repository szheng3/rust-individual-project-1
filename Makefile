rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
#	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1                                                    2 ✘  18s  16:40:49
#	export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo run -- text -i 'The Chinese monarchy collapsed in 1912 with the Xinhai Revolution, when the Republic of China (ROC) replaced the Qing dynasty. In its early years as a republic, the country underwent a period of instability known as the "Warlord Era" before mostly reunifying in 1928 under a Nationalist government. A civil war between the nationalist Kuomintang (KMT) and the Chinese Communist Party (CCP) began in 1927. Japan invaded China in 1937, starting the Second Sino-Japanese War and temporarily halting the civil war. The surrender and expulsion of Japanese forces from China in 1945 left a power vacuum in the country, which led to renewed fighting between the CCP and the Kuomintang.'

release:
	export LIBTORCH=/opt/homebrew/Cellar/pytorch/1.13.1 &&export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH&&cargo build --release

all: format lint test run
