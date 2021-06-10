import { useForm } from "react-hook-form";
export { Controller } from "react-hook-form";

type LoginInput = {
  username: string;
  password: string;
};

export const useLoginForm = () => useForm<LoginInput>();

type WorkflowInput = {
  name: string;
  slug: string;
  content: string;
}

export const useWorkflowForm = () => useForm<WorkflowInput>();
