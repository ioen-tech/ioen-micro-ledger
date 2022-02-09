FROM node

RUN mkdir /express
COPY . /express

WORKDIR /express

RUN npm install

CMD sleep 8 && node src/index.js