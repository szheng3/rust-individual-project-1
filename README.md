# IDS721 Spring 2023 indivusal Project 1 - Rust CLI Tool for machine learning: (summarization for the text)
[![Build Status](https://github.com/guillaume-be/rust-bert/workflows/Build/badge.svg?event=push)](https://github.com/szheng3/rust-individual-project-1/actions)
For this project, I interviewed a few people who are students. I found that the most common task that they do is to reading the books and summarize the text. So I decided to build a Rust Cli tool that can summarize the text. I used the [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
and libtorch to host a pretrained model in order to summarize the text.
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