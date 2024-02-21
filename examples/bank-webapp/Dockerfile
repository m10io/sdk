FROM node:18.17-bookworm
LABEL maintainer "M10 Networks, Inc."

EXPOSE 3000

WORKDIR /app

COPY ./services/bank-webapp/package.json package.json
COPY ./services/bank-webapp/yarn.lock yarn.lock

RUN yarn

COPY ./services/bank-webapp .

CMD yarn start -p 3000
