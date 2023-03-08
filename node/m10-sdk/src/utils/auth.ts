import superagent from "superagent";

import { M10Error } from "../error";

import { isNone, unwrap } from "./common";

export class TokenProvider {
    private address: string;
    private username: string;
    private password: string;
    private accessToken?: Option<string>;

    public constructor(username: string, password: string, address: string) {
        this.username = username;
        this.password = password;
        this.address = address;
    }

    public async getAccessToken(): Promise<string> {
        if (isNone(this.accessToken)) {

            const url = `https://${this.address}/oauth/token`;

            const response: superagent.Response = await superagent
                .post(url)
                .send({
                    "client_id": "directory",
                    "grant_type": "password",
                    "username": this.username,
                    "password": this.password,
                    "audience": "https://api.m10.net",
                    "scope": "offline_access openid",
                });

            this.accessToken = unwrap<string>(
                response?.body?.access_token,
                M10Error.Other("response?.body?.access_token is None"),
            );
        }
        return this.accessToken;
    }
}
