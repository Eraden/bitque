import useApi from 'shared/hooks/api';

const useCurrentUser = ({ cachePolicy = 'cache-only' } = {}) => {
  const res = useApi.get('/currentUser', {}, { cachePolicy });

  const [ { data } ] = res;
  return {
    currentUser: data && data.currentUser,
    currentUserId: data && data.currentUser ? data.currentUser.id : null,
  };
};

export default useCurrentUser;
