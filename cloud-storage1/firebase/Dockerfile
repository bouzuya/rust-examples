FROM node:20.10.0-bookworm
RUN apt-get update && apt-get install -y \
  openjdk-17-jre \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/* \
  && java --version \
  && npm install --global firebase-tools@13.0.2 \
  && firebase --version
RUN useradd --create-home bouzuya
USER bouzuya
RUN mkdir -p /home/bouzuya/cloud-storage1
WORKDIR /home/bouzuya/cloud-storage1/firebase
CMD ["firebase", "emulators:start"]
