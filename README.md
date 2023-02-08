[![Tests](https://github.com/szheng3/rust-individual-project-1/actions/workflows/tests.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-1/actions/workflows/tests.yml)
[![Build binary release](https://github.com/szheng3/rust-individual-project-1/actions/workflows/release.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-1/actions/workflows/release.yml)
[![Clippy](https://github.com/szheng3/rust-individual-project-1/actions/workflows/lint.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-1/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/szheng3/rust-individual-project-1/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-1/actions/workflows/rustfmt.yml)
[![publish to Dockerhub](https://github.com/szheng3/rust-individual-project-1/actions/workflows/publish.yml/badge.svg)](https://github.com/szheng3/rust-individual-project-1/actions/workflows/publish.yml)

# IDS721 Spring 2023 Individual Project 1 - Rust CLI Tool for Text Summarization

This project aims to build a Rust CLI tool that summarizes text, based on the common task of reading and summarizing books among students. The project uses the `rust-cli-template` and `libtorch` to host a pre-trained model for summarization.

![image](./assets/ml.png)

## Project Goals/Outcomes

* Develop my first Rust project
* Use Github Codespaces and Copilot
* Integrate libtorch and 'hugging-face pretrained models' into a Rust Cli project

## Setup

1. Create an Rust environments with cargo
2. Install the libtorch (for Mac M1), Intel chips users can skip this step
```
$ brew install pytorch@1.13.1
```


## Run

**intel chips users**
```
$ make run PARAMETER='The Chinese monarchy collapsed in 1912 with the Xinhai Revolution, when the Republic of China (ROC) replaced the Qing dynasty. In its early years as a republic, the country underwent a period of instability known as the \"Warlord Era\" before mostly reunifying in 1928 under a Nationalist government. A civil war between the nationalist Kuomintang (KMT) and the Chinese Communist Party (CCP) began in 1927. Japan invaded China in 1937, starting the Second Sino-Japanese War and temporarily halting the civil war. The surrender and expulsion of Japanese forces from China in 1945 left a power vacuum in the country, which led to renewed fighting between the CCP and the Kuomintang.'

```

**M1 chips users**
```
$ make runarm PARAMETER='The Chinese monarchy collapsed in 1912 with the Xinhai Revolution, when the Republic of China (ROC) replaced the Qing dynasty. In its early years as a republic, the country underwent a period of instability known as the \"Warlord Era\" before mostly reunifying in 1928 under a Nationalist government. A civil war between the nationalist Kuomintang (KMT) and the Chinese Communist Party (CCP) began in 1927. Japan invaded China in 1937, starting the Second Sino-Japanese War and temporarily halting the civil war. The surrender and expulsion of Japanese forces from China in 1945 left a power vacuum in the country, which led to renewed fighting between the CCP and the Kuomintang.'

```

*Below is the screenshot for the results*
![results](./assets/result.png)


## CI/CD

Github Actions configured in .github/workflows

**Build Executable**
```
$ make release
```


## Progress Log

- [x] Configure Github Codespaces.
- [x] Initialise Rust project with pretrained model from [hugging-face](https://huggingface.co/transformers/model_doc/bart.html)
- [x] Add clap command line parsing for arguments (Text)
- [x] Dockerize the project.
- [x] CI/CD with Github Actions

## References


* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-bert](https://github.com/guillaume-be/rust-bert)