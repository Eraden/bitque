import { request } from './index';

export const currentUser = () =>
    request({ path: '/currentUser' });
