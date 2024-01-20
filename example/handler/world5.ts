import { Model, View } from "afast"
import { User } from "../model/user"

export class UserView implements View {
    model: typeof Model = User
    allowed: string[] = ['GET', 'POST', 'PUT', 'DELETE']
}
