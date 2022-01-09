FROM node:16.8-stretch

WORKDIR /usr/src/app

COPY package-lock.json ./
COPY package.json ./
RUN npm ci

COPY . .

RUN npm run build

CMD [ "npm", "start" ]
