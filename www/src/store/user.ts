import create from 'zustand'
import axios from 'axios';

type User = {
  username: string | null;
  email: string | null;
  loading: boolean;
  set: (user: User) => void;
  fetch: () => Promise<void>;
}

const useUserStore = create<User>(set => ({
  username: null,
  email: null,
  loading: false,
  set: (user: User) => set(user),
  fetch: async () => {
    try {
      set({ loading: true });
      const { data } = await axios.get<User>('http://localhost:3001/users/me');
      set({ ...data, loading: false })
    } catch { }
    finally {
      set({ loading: false })
    }
  }
}))

export default useUserStore;
