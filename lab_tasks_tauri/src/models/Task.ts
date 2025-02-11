export interface Task {
  id: number;
  name: string;
  description: string;
  category: string;
  status: string;
  created_at: Date;
  updated_at: Date;
  completed: boolean;
  archived: boolean;
  user: number;
  deadline: Date;
  tel: string;
}

export interface TaskDTO
  extends Omit<Task, "created_at" | "updated_at" | "completed" | "archived"> {
  created_at: string;
  updated_at: string;
  completed: number;
  archived: number;
}

export const mapTaskFromDTO = (task: TaskDTO): Task => {
  return {
    ...task,
    created_at: new Date(task.created_at),
    updated_at: new Date(task.updated_at),
    archived: task.archived === 1,
    completed: task.completed === 1,
  };
};

export const mapTaskToDTO = (task: Task): TaskDTO => {
  return {
    ...task,
    created_at: task.created_at.toISOString(),
    updated_at: task.updated_at.toISOString(),
    archived: task.archived ? 1 : 0,
    completed: task.completed ? 1 : 0,
  };
};
