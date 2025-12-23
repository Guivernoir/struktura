import React, { useState } from "react";
import PropTypes from "prop-types";
import Icon from "../Icon";
import InputField from "./InputField"; // Use the updated InputField

/**
 * ArrayInput - Manages dynamic lists of objects.
 * Updated to use our standard InputField for consistency and
 * to support the new DateTimePicker automatically.
 */
const ArrayInput = ({
  label,
  name,
  value = [],
  onChange,
  itemSchema,
  required = false,
  helpText,
  t,
}) => {
  const [expandedItems, setExpandedItems] = useState(new Set([0]));

  const toggleItem = (index) => {
    setExpandedItems((prev) => {
      const next = new Set(prev);
      next.has(index) ? next.delete(index) : next.add(index);
      return next;
    });
  };

  const addItem = () => {
    const newItem = {};
    itemSchema.fields.forEach((field) => {
      newItem[field.name] = field.default !== undefined ? field.default : "";
    });

    const newArray = [...value, newItem];
    onChange({ target: { name, value: newArray } });

    // Auto-expand the new item
    setExpandedItems((prev) => new Set([...prev, newArray.length - 1]));
  };

  const removeItem = (index) => {
    const newArray = value.filter((_, i) => i !== index);
    onChange({ target: { name, value: newArray } });

    // Adjust expansion indices
    setExpandedItems((prev) => {
      const next = new Set();
      prev.forEach((i) => {
        if (i < index) next.add(i);
        else if (i > index) next.add(i - 1);
      });
      return next;
    });
  };

  const updateItemField = (index, fieldName, fieldValue) => {
    const newArray = [...value];
    newArray[index] = { ...newArray[index], [fieldName]: fieldValue };
    onChange({ target: { name, value: newArray } });
  };

  return (
    <div className="space-y-3 w-full">
      {/* Header */}
      <div className="flex items-center justify-between">
        <label className="text-sm font-bold text-charcoal-700 dark:text-steel-300">
          {label}
          {required && <span className="text-red-500 ml-1">*</span>}
        </label>
        <button
          type="button"
          onClick={addItem}
          className="flex items-center gap-1.5 px-3 py-1.5 text-xs font-bold text-indigo-600 dark:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-900/20 rounded-lg border border-transparent hover:border-indigo-200 transition"
        >
          <Icon name="Plus" size={14} />
          {t?.engineer?.form?.add_item || "Add Item"}
        </button>
      </div>

      {helpText && (
        <p className="text-xs text-charcoal-500 dark:text-steel-500">
          {helpText}
        </p>
      )}

      {/* List of Items */}
      <div className="space-y-3">
        {value.length === 0 ? (
          <div className="p-8 border-2 border-dashed border-sand-300 dark:border-charcoal-700 rounded-xl text-center">
            <p className="text-sm text-charcoal-400 dark:text-steel-500">
              {t?.engineer?.form?.no_items || "No items added yet."}
            </p>
          </div>
        ) : (
          value.map((item, index) => (
            <div
              key={index}
              className="border border-sand-300 dark:border-charcoal-700 rounded-xl bg-white dark:bg-charcoal-900/50 shadow-sm overflow-hidden"
            >
              {/* Accordion Header */}
              <div className="flex items-center justify-between px-4 py-2.5 bg-sand-50 dark:bg-charcoal-800/40 border-b border-sand-200 dark:border-charcoal-800">
                <button
                  type="button"
                  onClick={() => toggleItem(index)}
                  className="flex items-center gap-2 text-sm font-semibold text-charcoal-700 dark:text-steel-200"
                >
                  <Icon
                    name={
                      expandedItems.has(index) ? "ChevronDown" : "ChevronRight"
                    }
                    size={16}
                    className="text-indigo-500"
                  />
                  {t?.engineer?.form?.item || "Item"} {index + 1}
                </button>
                <button
                  type="button"
                  onClick={() => removeItem(index)}
                  className="p-1.5 text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition"
                  title="Remove Item"
                >
                  <Icon name="Trash2" size={16} />
                </button>
              </div>

              {/* Accordion Body */}
              {expandedItems.has(index) && (
                <div className="p-4 grid grid-cols-1 gap-4 animate-in slide-in-from-top-1 duration-200">
                  {itemSchema.fields.map((field) => (
                    <InputField
                      key={field.name}
                      label={field.label}
                      name={field.name}
                      // Map "datetime" from schema to "datetime-local" for InputField logic
                      type={
                        field.type === "datetime"
                          ? "datetime-local"
                          : field.type
                      }
                      value={item[field.name]}
                      unit={field.unit}
                      required={field.required}
                      placeholder={field.placeholder}
                      helpText={field.helpText}
                      step={field.step}
                      options={field.enum?.map((opt) => ({
                        value: opt,
                        label: opt
                          .replace(/_/g, " ")
                          .replace(/\b\w/g, (l) => l.toUpperCase()),
                      }))}
                      onChange={(e) =>
                        updateItemField(index, field.name, e.target.value)
                      }
                    />
                  ))}
                </div>
              )}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

ArrayInput.propTypes = {
  label: PropTypes.string.isRequired,
  name: PropTypes.string.isRequired,
  value: PropTypes.array,
  onChange: PropTypes.func.isRequired,
  itemSchema: PropTypes.shape({
    fields: PropTypes.arrayOf(
      PropTypes.shape({
        name: PropTypes.string.isRequired,
        label: PropTypes.string.isRequired,
        type: PropTypes.string.isRequired,
        unit: PropTypes.string,
        required: PropTypes.bool,
        enum: PropTypes.arrayOf(PropTypes.string),
        default: PropTypes.any,
        placeholder: PropTypes.string,
        helpText: PropTypes.string,
        step: PropTypes.string,
      })
    ).isRequired,
  }).isRequired,
  required: PropTypes.bool,
  helpText: PropTypes.string,
  t: PropTypes.object,
};

export default ArrayInput;
