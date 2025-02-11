import { Task } from "../../models/Task";

interface TaskDetailsProps {
  task: Task;
  closeDetails: (open: boolean) => void;
}
export const TaskDetails = (props: TaskDetailsProps) => {
  return (
    <div
      className="w-[100%] h-[100%] min-h-[100%] min-w-[100%] -translate-y-1/2 -translate-x-1/2 fixed top-1/2 left-1/2 bg-opacity-50 z-50 flex flew-row align-middle justify-center items-center backdrop-blur-sm drop-shadow-lg"
      onClick={(e) => {
        e.stopPropagation();
        props.closeDetails(false);
      }}
    >
      <div
        className="flex flex-col bg-white text-black p-4 rounded-2xl"
        onClick={(e) => e.stopPropagation()}
      >
        <p>{props.task.name}</p>
        <p>{props.task.description}</p>
        <p>{props.task.tel}</p>
        <p>{new Date(props.task.deadline).toLocaleDateString()}</p>
      </div>
    </div>
  );
};
