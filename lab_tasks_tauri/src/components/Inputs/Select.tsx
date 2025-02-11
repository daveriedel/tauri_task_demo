interface SelectProps {
  options: Array<{ value: string; key?: string }>;
  value?: string;
  label: string;
  onChange: (value: string) => void;
}
export const Select = (props: SelectProps) => {
  return (
    <>
      <span className="flex flex-col items-start">
        <p className="text-gray-600">{props.label}:</p>
        <select
          className="text-black  p-2 rounded-md w-full border-1 border-gray-300"
          value={props.value}
          onChange={(e) => props.onChange(e.target.value)}
        >
          {props.options.map((option, index) => (
            <option key={index}>{option.value}</option>
          ))}
        </select>
      </span>
    </>
  );
};
