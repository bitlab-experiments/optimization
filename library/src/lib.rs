use neon::prelude::*;

struct Model {
    pub name: String,
    pub alpha: f64,
    pub beta: f64,
    pub a: Vec<Vec<f64>>,
    pub b: Option<Vec<f64>>,
    pub c: Vec<f64>,
}

impl Model {
    fn calc_score<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsNumber> {
        let mut sum_first_term: f64 = 0.0;
        let mut sum_second_term: f64 = 0.0;
        let b = self.b.clone().unwrap();
        for (i, ai) in self.a.iter().enumerate() {
            for (j, aij) in ai.iter().enumerate() {
                sum_first_term += aij * (b[i] * b[j]);
            }
            sum_second_term += self.c[i] * b[i];
        }

        let result = (self.alpha * sum_first_term) + (self.beta * sum_second_term);
        Ok(cx.number(result))
    }

    fn to_object<'a>(&self, cx: &mut impl Context<'a>) -> JsResult<'a, JsObject> {
        let obj = cx.empty_object();

        let name = cx.string(&self.name);
        obj.set(cx, "name", name)?;

        let alpha = cx.number(self.alpha);
        obj.set(cx, "alpha", alpha)?;

        let beta = cx.number(self.beta);
        obj.set(cx, "beta", beta)?;

        let a = vec_to_vec_array(&self.a, cx)?;
        obj.set(cx, "a", a)?;

        let b = vec_to_array(&self.b.clone().unwrap(), cx)?;
        obj.set(cx, "b", b)?;

        let c = vec_to_array(&self.c, cx)?;
        obj.set(cx, "c", c)?;

        let score = self.calc_score(cx)?;
        obj.set(cx, "score", score)?;

        Ok(obj)
    }
}

// Helper function to convert Vec<f64> to JS array
fn vec_to_array<'a, C: Context<'a>>(vec: &Vec<f64>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = cx.number(*s);
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

// Helper function to convert Vec<Vec<f64>> to JS array
fn vec_to_vec_array<'a, C: Context<'a>>(vec: &Vec<Vec<f64>>, cx: &mut C) -> JsResult<'a, JsArray> {
    let a = JsArray::new(cx, vec.len() as usize);

    for (i, s) in vec.iter().enumerate() {
        let v = vec_to_array(s, cx)?;
        a.set(cx, i as u32, v)?;
    }

    Ok(a)
}

// Helper function to convert JS array to Vec<f64>
fn js_array_to_vec_f64<'a>(cx: &mut FunctionContext<'a>, arr: Handle<'a, JsArray>) -> NeonResult<Vec<f64>> {
    let len = arr.len(cx);
    let mut vec = Vec::with_capacity(len as usize);
    
    for i in 0..len {
        let val: Handle<JsNumber> = arr.get(cx, i)?;
        vec.push(val.value(cx));
    }
    
    Ok(vec)
}

// Helper function to convert JS array to Vec<Vec<f64>> (2D array)
fn js_array_to_vec_vec_f64<'a>(cx: &mut FunctionContext<'a>, arr: Handle<'a, JsArray>) -> NeonResult<Vec<Vec<f64>>> {
    let len = arr.len(cx);
    let mut vec = Vec::with_capacity(len as usize);
    
    for i in 0..len {
        let inner_arr: Handle<JsArray> = arr.get(cx, i)?;
        let inner_vec = js_array_to_vec_f64(cx, inner_arr)?;
        vec.push(inner_vec);
    }
    
    Ok(vec)
}

#[neon::export]
fn create_model<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsObject> {
    let alpha = cx.argument::<JsNumber>(0)?.value(cx);
    let beta = cx.argument::<JsNumber>(1)?.value(cx);
    let a = cx.argument::<JsArray>(2)?;
    let b = cx.argument::<JsArray>(3)?;
    let c = cx.argument::<JsArray>(4)?;

    let model = Model {
        name: "Test".to_string(),
        alpha: alpha,
        beta: beta,
        a: js_array_to_vec_vec_f64(cx, a)?,
        b: Some(js_array_to_vec_f64(cx, b)?),
        c: js_array_to_vec_f64(cx, c)?,
    };

    model.to_object(cx)
}