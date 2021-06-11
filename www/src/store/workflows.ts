import create from 'zustand'
import axios from 'axios';

type Workflow = {
  name: string;
  slug: string;
  secret: string;
  content: string;
}

type Workflows = {
  items: Workflow[]
};

interface WorkflowHandler extends Workflows {
  loading: boolean;
  insert: (workflow: Workflow) => Promise<any>;
  get: () => Promise<any>;
}

export const useWorkflowStore = create<WorkflowHandler>(set => ({
  items: [],
  loading: false,
  insert: async () => null,
  get: async () => {
    set({ loading: true });
    const { data } = await axios.get<Workflow[]>('/workflows');
    set({ items: data })
    set({ loading: false })
  }
}))

export default useWorkflowStore;
