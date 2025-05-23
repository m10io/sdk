FROM node:18-slim
LABEL maintainer="M10 Networks, Inc."

# Set the Node environment to production for security and performance
ENV NODE_ENV=production

# Create a non-root user and group
RUN addgroup --system appgroup && adduser --system appuser --ingroup appgroup

WORKDIR /app

# Copy only package.json and yarn.lock first to leverage Docker build cache
COPY ./services/bank-webapp/package.json ./services/bank-webapp/yarn.lock ./

# Install only production dependencies
RUN yarn install --frozen-lockfile --production --prefer-offline --no-progress && \
    yarn cache clean

# Copy the rest of the application code
COPY ./services/bank-webapp/ .

# Ensure correct ownership
RUN chown -R appuser:appgroup /app

# Switch to the non-root user
USER appuser

# Expose the application port
EXPOSE 3000

# Run the application
CMD ["yarn", "start", "-p", "3000"]
