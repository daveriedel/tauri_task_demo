import { invoke } from "@tauri-apps/api/core";
import { useContext, useEffect, useState } from "react";
import { TaskList } from "../../components/Task/TaskList";
import { Task } from "../../models/Task";
import { getTasks, putTask } from "../../services/taskService";
import { TaskContext } from "../../components/Task/TaskContext";
interface FileResponse {
  files: Array<string>;
}
export const TaskPage = () => {
  const [files, setFiles] = useState<Array<string>>([]);
  const { tasks, setTasks } = useContext(TaskContext);

  useEffect(() => {
    const fetchTasks = async () => {
      setTasks(await getTasks());
    };
    fetchTasks();
  }, []);

  const onUpdatedTask = async (task: Task) => {
    const prevTasks = tasks;
    const updatedTasks = prevTasks.map((t) => {
      if (t.id === task.id) {
        return task;
      }
      return t;
    });
    setTasks(updatedTasks);

    await putTask(task);
  };

  function handleFilePicker() {
    invoke("open_file_picker").then((value: unknown) => {
      let result = value as FileResponse;
      setFiles(result.files);
    });
  }

  return (
    <>
      <div>
        <TaskList
          tasks={tasks.filter((task) => !task.completed)}
          onUpdateTask={onUpdatedTask}
          title="Open:"
        />
        <span className="mt-4"></span>
        <TaskList
          tasks={tasks.filter((task) => task.completed)}
          onUpdateTask={onUpdatedTask}
          title="Completed:"
        />
        <h2 className="mt-16">File demo</h2>
        <button onClick={() => handleFilePicker()}>Click me!</button>
        {files.map((file, index) => (
          <p key={index}>{file}</p>
        ))}
      </div>
    </>
  );
};
