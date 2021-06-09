import create from 'zustand'
import axios from 'axios';

type User = {
  username: string | null;
  email: string | null;
}

type UserHandler = {
  loading: boolean;
  set: (user: User) => void;
  fetch: () => Promise<void>;
  auth: (user: { username: string; password: string }) => Promise<void>;
}

const useUserStore = create<User & UserHandler>(set => ({
  username: null,
  email: null,
  loading: false,
  set: (user: User) => set(user),
  fetch: async () => {
    try {
      set({ loading: true });
      const { data } = await axios.get<User>('/users/me');
      set(data)
    } catch { }
    finally {
      set({ loading: false })
    }
  },
  auth: async (user: { username: string; password: string }) => {
    try {
      set({ loading: true });
      const { data } = await axios.post<User>('/auth/login', user, {
        headers: {
          'content-type': 'application/json'
        }
      });
      set(data)
    } catch { }
    finally {
      set({ loading: false })
    }
  }
}))

export default useUserStore;
