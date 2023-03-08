FROM cirrusci/flutter:2.2.3

WORKDIR /app

COPY ./shared/flutter/m10_security ./shared/flutter/m10_security
COPY ./shared/flutter/m10_sdk ./shared/flutter/m10_sdk

RUN sudo chown -R cirrus:cirrus /app

WORKDIR /app/shared/flutter/m10_security
RUN flutter pub get

WORKDIR /app/shared/flutter/m10_sdk
RUN flutter pub get

RUN flutter test ./test/user_test.dart
