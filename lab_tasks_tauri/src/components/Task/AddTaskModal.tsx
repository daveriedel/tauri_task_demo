import { useContext, useState } from "react";
import { Task } from "../../models/Task";
import { postTask } from "../../services/taskService";
import { Button } from "../Inputs/Button";
import { Input } from "../Inputs/Input";
import { Select } from "../Inputs/Select";
import { TaskContext } from "./TaskContext";

interface AddTaskModalProps {
  onClose: () => void;
  onSave: () => void;
}
export const AddTaskModal = (props: AddTaskModalProps) => {
  const taskCategories = ["Home", "Work", "Call"];
  const { tasks, setTasks } = useContext(TaskContext);
  const [task, setTask] = useState<Task>({
    id: 0,
    name: "",
    description: "",
    category: "Work",
    created_at: new Date(),
    updated_at: new Date(),
    completed: false,
    archived: false,
    status: "",
    user: 0,
    deadline: new Date(),
    tel: "",
  });

  const updateTaskField = (field: keyof Task, value: string | Date) => {
    let taskCopy = { ...task };
    taskCopy[field] = value;
    setTask(taskCopy);
  };

  return (
    <>
      <div className="w-[100%] h-[100%] min-h-[100%] min-w-[100%] -translate-y-1/2 -translate-x-1/2 fixed top-1/2 left-1/2 bg-opacity-50 z-50 flex flew-row align-middle justify-center items-center backdrop-blur-sm drop-shadow-lg">
        <div className="flex flex-col mb-16 rounded-xl w-fit h-fit bg-white p-8 border-gray-200 border">
          <div className="flex flex-col justify-center items-center">
            <h1 className="text-lapis-lazuli-200 text-start w-full ml-2">
              Add a task
            </h1>
            <div className="flex flex-col mt-2 w-full">
              <Input
                label="Task name"
                type="text"
                value={task.name}
                onChange={(value) => {
                  console.log(value);
                  updateTaskField("name", value);
                }}
              />
              {task.category === "Call" && (
                <Input
                  label="Tel nr"
                  type="text"
                  value={task.tel}
                  onChange={(value) => {
                    console.log(value);
                    updateTaskField("tel", value);
                  }}
                />
              )}
              <Input
                label="Task description"
                type="textarea"
                value={task.description}
                onChange={(value) => {
                  console.log(value);
                  updateTaskField("description", value);
                }}
              />

              <Select
                label="Task category"
                value={task.category}
                options={taskCategories.map((category) => ({
                  value: category,
                }))}
                onChange={(value) => {
                  updateTaskField("category", value);
                }}
              />
              <Input
                label="Deadline"
                type="text"
                value={task.deadline.toLocaleDateString()}
                onChange={(value) => {
                  const date = new Date(value);
                  updateTaskField("deadline", date);
                }}
              />
            </div>
          </div>
          <div className="flex flex-row mt-4">
            <Button
              text="save"
              color=""
              onClick={() => {
                postTask(task);
                const newTasks = [...tasks, task];
                setTasks(newTasks);
                props.onClose();
              }}
            />
            <Button
              text="cancel"
              color="error"
              onClick={() => props.onClose()}
            />
          </div>
        </div>
      </div>
    </>
  );
};
