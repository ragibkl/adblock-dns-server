FROM node

WORKDIR /app

COPY package.json package.json
COPY yarn.lock yarn.lock
RUN yarn install --production

COPY src/ .

CMD NODE_ENV=production node index.js
