import { createContext, Dispatch, SetStateAction } from "react";
import { Task } from "../../models/Task";

export const TaskContext = createContext<{
  tasks: Array<Task>;
  setTasks: Dispatch<SetStateAction<Array<Task>>>;
}>({
  tasks: [],
  setTasks: () => {},
});
