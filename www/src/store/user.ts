import create from 'zustand'

type User = {
  username: string | null
}

const useUserStore = create<User>(set => ({
  username: null,
  set: (user: User) => set(user),
}))

export default useUserStore;
