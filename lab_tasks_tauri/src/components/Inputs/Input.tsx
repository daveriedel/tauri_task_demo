interface InputProps {
  label: string;
  value: string;
  type: string;
  onChange: (value: string) => void;
}
export const Input = (props: InputProps) => {
  return (
    <>
      <span className="flex flex-col items-start w-full my-2">
        <p className="text-gray-600">{props.label}:</p>
        {props.type === "text" ? (
          <input
            type="text"
            value={props.value}
            className="p-2 text-gray-400 w-full border-1 rounded-md border-gray-300"
            onChange={(e) => props.onChange(e.target.value)}
          />
        ) : props.type === "textarea" ? (
          <textarea
            value={props.value}
            className="p-2 text-gray-400 w-full border-1 rounded-md border-gray-300"
            onChange={(e) => props.onChange(e.target.value)}
          />
        ) : (
          <></>
        )}
      </span>
    </>
  );
};
