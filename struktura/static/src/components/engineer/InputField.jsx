import PropTypes from "prop-types";

const InputField = ({
  label,
  name,
  type = "number",
  unit,
  value,
  onChange,
  min = 0.1,
  step = "0.1",
}) => (
  <div className="flex flex-col space-y-1">
    <label
      htmlFor={name}
      className="text-sm font-medium text-charcoal-700 dark:text-steel-300"
    >
      {label}
    </label>
    <div className="relative">
      <input
        id={name}
        name={name}
        type={type}
        min={min}
        step={step}
        value={value}
        onChange={onChange}
        // UPDATED: Changed focus ring/border to indigo-500
        className="w-full p-3 pr-10 border border-sand-300 dark:border-charcoal-700 bg-white dark:bg-charcoal-800 rounded-xl text-charcoal-900 dark:text-white focus:ring-indigo-500 focus:border-indigo-500 transition"
      />
      {unit && (
        <span className="absolute right-3 top-1/2 -translate-y-1/2 text-sm text-charcoal-500 dark:text-steel-500 font-medium">
          {unit}
        </span>
      )}
    </div>
  </div>
);

InputField.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  type: PropTypes.string,
  unit: PropTypes.string,
  value: PropTypes.oneOfType([PropTypes.string, PropTypes.number]),
  onChange: PropTypes.func.isRequired,
  min: PropTypes.number,
  step: PropTypes.string,
};

export default InputField;
