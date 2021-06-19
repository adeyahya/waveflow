import create from "zustand";
import axios from "axios";

type History = {
  id: string;
  name: string;
  slug: string;
  secret: string;
  content: string;
};

type Histories = {
  items: Record<string, History[]>;
};

interface HistoryHandler extends Histories {
  loading: boolean;
  get: (slug: string) => Promise<any>;
}

export const useWorkflowHistoryStore = create<HistoryHandler>((set) => ({
  items: {},
  loading: false,
  get: async (id: string) => {
    set({ loading: true });
    const { data } = await axios.get<History[]>(`/workflows/${id}/history`);
    set({
      items: {
        [id]: data
      }
    });
    set({ loading: false });
  },
}));

export default useWorkflowHistoryStore;
