FROM ghcr.io/cirruslabs/flutter:3.29.2

WORKDIR /app

USER cirrus

COPY ./shared/flutter/m10_security ./shared/flutter/m10_security
COPY ./shared/flutter/m10_sdk ./shared/flutter/m10_sdk

WORKDIR /app/shared/flutter/m10_security
RUN flutter pub get

WORKDIR /app/shared/flutter/m10_sdk
RUN flutter pub get

RUN flutter test ./test/user_test.dart
