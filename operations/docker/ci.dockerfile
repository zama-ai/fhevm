FROM node:20

# Set the working directory inside the container
WORKDIR /app
COPY package.json ./

# Install the dependencies
RUN npm install

COPY .env.example.deployment ./
COPY lib ./lib/
COPY tasks ./tasks/
COPY gateway ./gateway/
COPY *.sh ./
COPY *.ts ./
COPY tsconfig.json ./
COPY *.sh ./