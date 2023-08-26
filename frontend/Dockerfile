# Build
FROM node:18-alpine as builder

RUN npm i -g pnpm

WORKDIR /panel
COPY . .

RUN pnpm install --frozen-lockfile
RUN pnpm build


# Copy from builder and run
FROM node:18-alpine

COPY --from=builder /panel/package.json .
COPY --from=builder /panel/build .

EXPOSE 5173

CMD ["node", "index.js"]

