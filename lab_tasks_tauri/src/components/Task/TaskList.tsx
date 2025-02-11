import { Task } from "../../models/Task";
import { TaskRow } from "./TaskRow";

interface TaskListProps {
  title: string;
  tasks: Array<Task>;
  onUpdateTask: (task: Task) => void;
}
export const TaskList = (props: TaskListProps) => {
  return (
    <div className="w-full flex flex-col items-start">
      <h2 className="ml-4">{props.title}</h2>
      {props.tasks.length > 0 &&
        props.tasks.map((task) => (
          <TaskRow task={task} onCheckTask={props.onUpdateTask} />
        ))}
    </div>
  );
};
