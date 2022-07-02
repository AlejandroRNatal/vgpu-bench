FROM rust:1.62

WORKDIR /usr/src/vgpu-bench
COPY . .

# RUN cargo install --path .
RUN apt-get update -y && apt install -y --no-install-recommends libgtk-3-dev

RUN apt-get update -y && \
     DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
         apt-transport-https \
         ca-certificates \
         gnupg \
         wget && \
     rm -rf /var/lib/apt/lists/*

RUN wget -qO - https://developer.download.nvidia.com/devtools/repos/ubuntu2004/amd64/nvidia.pub | apt-key add - && \
     echo "deb https://developer.download.nvidia.com/devtools/repos/ubuntu2004/amd64/ /" >> /etc/apt/sources.list.d/nsight.list && \
     apt-get update -y && \
     DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
         nsight-systems-2020.2.1 && \
     rm -rf /var/lib/apt/lists/*

# RUN apt-get install -y cuda-nsight-systems-10-2 nsight-systems-2019.5.2

# Enable IP sampling inside container
# RUN sh -c 'echo 2 >/proc/sys/kernel/perf_event_paranoid'

#Check that NSYS installed correctly
RUN echo nsys status -e

RUN cargo test 