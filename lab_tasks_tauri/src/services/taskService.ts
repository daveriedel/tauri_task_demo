import { Task } from "../models/Task";
const baseUrl = import.meta.env.VITE_BASE_URL;
export const postTask = async (task: Task) => {
  await fetch(`${baseUrl}/tasks`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(task),
  });
};

export const putTask = async (task: Task) => {
  await fetch(`${baseUrl}/tasks`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(task),
  });
};

export const getTasks = async () => {
  const task_req = await fetch(`${baseUrl}/tasks`, {
    headers: { "Content-Type": "application/json" },
  });
  const json = await task_req.json();
  return JSON.parse(json.result) as Array<Task>;
};
