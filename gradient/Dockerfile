FROM node:16.13.1-alpine as build

ARG REACT_APP_API_BASE_URL
ARG REACT_APP_API_KEY

ENV REACT_APP_API_BASE_URL=$REACT_APP_API_BASE_URL
ENV REACT_APP_API_KEY=$REACT_APP_API_KEY

WORKDIR /app
COPY package.json ./
COPY package-lock.json ./
RUN npm ci
COPY . ./
RUN npm run build

FROM nginx:stable-alpine
COPY --from=build /app/build /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
