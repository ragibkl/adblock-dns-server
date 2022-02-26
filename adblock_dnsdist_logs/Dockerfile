FROM node:16

WORKDIR /app

COPY package.json yarn.lock ./
RUN yarn install --production

COPY src/ src

CMD NODE_ENV=production node src/index.js
