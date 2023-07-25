import {$, component$} from '@builder.io/qwik';
import type { Item } from '~/types';
import type { ItemOptions } from '~/components/stash/cell/stash-cell';
import { routeLoader$, z } from '@builder.io/qwik-city';
import type { InitialValues, SubmitHandler } from '@modular-forms/qwik';
import { formAction$, useForm, zodForm$ } from '@modular-forms/qwik';
import style from './dialog.module.css';

interface DialogProps {
    item: Item;
    option: ItemOptions;
}

const moneySchema = z.object({
    amount: z
        .number()
        .min(1, '1 is min')
        .max(500000, '500.000 is max'),
});

type MoneyForm = z.infer<typeof moneySchema>;


export const Dialog = component$<DialogProps>(({ item, option }: DialogProps) => {

    const useFormLoader = routeLoader$<InitialValues<MoneyForm>>(() => ({
        amount: item.amount,
    }));

    const useFormAction = formAction$<MoneyForm>((values) => {
        // Runs on server
    }, zodForm$(moneySchema));
    const [_loginForm, { Form, Field }] = useForm<MoneyForm>({
        loader: useFormLoader(),
        action: useFormAction(),
        validate: zodForm$(moneySchema),
    });

    if (option !== 'change_amount') {
        return <h3>Option not supported {option}</h3>;
    }

    const handleSubmit = $((values, event) => {
        // Runs on client
        console.log(values);
    });

    return (
        <div class={style.dialog}>
            <Form onSubmit$={handleSubmit}>
                <Field name="amount" type="number">
                    {(field, props) => (
                        <div>
                            <input {...props} value={field.value} />
                            {field.error && <div>{field.error}</div>}
                        </div>
                    )}
                </Field>

                <button type="submit">Update</button>
            </Form>
        </div>
    );
});
