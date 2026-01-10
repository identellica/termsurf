FROM oven/bun:1 AS base
WORKDIR /app

# Install dependencies
FROM base AS deps
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile

# Build client assets only
FROM base AS build
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN bun run build

# Production - run TypeScript directly
FROM base AS production
WORKDIR /app
COPY --from=build /app/dist/client ./dist/client
COPY --from=build /app/node_modules ./node_modules
COPY --from=build /app/package.json ./
COPY --from=build /app/server.tsx ./
COPY --from=build /app/src ./src
COPY --from=build /app/tsconfig.json ./
COPY --from=build /app/data ./data

ENV NODE_ENV=production
ENV PORT=3000
EXPOSE 3000

CMD ["bun", "run", "server.tsx"]
