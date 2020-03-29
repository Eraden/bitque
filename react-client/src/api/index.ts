import { getStoredAuthToken } from 'shared/utils/authToken';
import axios from 'axios';

export interface RequestBody {
    path: string,
    query?: string,
    body?: object,
    form?: FormData,
    method?: string,
}

export const endpoint = (): string => `http://localhost:3000`;

export const getContentType = (method, form) =>
    method === 'GET' || form instanceof FormData
        ? ({})
        : ({ 'Content-Type': 'application/json' });

export const buildHeaders = (method, form) => ({
    'Access-Control-Allow-Origin': '*',
    Authorization: `Bearer ${ getStoredAuthToken() }`,
    ...getContentType(method, form),
});

export const client = (method, ...argv) => axios.create(...argv);

export const request = ({ path, query = '', body, form, method = 'string' }: RequestBody) =>
    method === 'GET'
        ? client(method, `${ endpoint() }${ path }${ query }`, {
            method,
            headers: buildHeaders(method, form),
        })
        : client(method, `${ endpoint() }${ path }${ query }`, {
            method,
            body: body ? JSON.stringify(body) : form,
            headers: buildHeaders(method, form),
        });
